---
title: "It's 2024 and This Blog is Now On The Edge"
date: 2024-01-05T09:18:47-05:00
draft: true
toc: true
images:
tags: 
  - untagged
---

#  I'm Six Feet From The Edge and I'm Thinking... 

Well, unlike that [Classic Creed Song](https://www.youtube.com/watch?v=qnkuBUAwfe0) _One Last Breath_ that was on the radio when I was a kid I'm actually on the edge! However, not in the way the song was intended, definitely in a technical sense.

I am taking advantage of learning at my new job by utitilzing [Fastly's](htttps://fastly.com) Edge Compute service and deploying this blog from their edge network making this site 100% delievered from a WASM pre-compiled binary. In this post I'll be explaining how I did it along with some lessons I learned along the way. 

## Why I Put The Blog On The Edge

Previously, for about 10 years I have been using the same [Digital Ocean](https://digitalocean.com) Droplet as a Virtual Private Server (VPS) serving my site. I have no complaints and it's only costed me about $7/month to have a website that I can do almost anything with! However, it's gotten pretty out-of-date on the backend and I did not have a real good way to fully automate a deployment without doing (in my opinion) a lot of work. Plus, a new friend at work saw my certificates were expried again and pushed me to start digging more into my new company's offerings (thanks Colton!).

### But What Advantage Does The Edge Have? 

Many people will be able to describe this better than me, but for me the main advantage is freeing up my VPS for updating and more experiments and hosting a faster blog around the world. 

The Edge is essentially a CDN but instead of content sitting in cache around the world it is computed on the fly when requested at the site that's closest. Both a CDN and the Edge offer faster websites and I wanted to move closer to one repository and deploy method for the site. That and since my site is _so simple_ I opted for compute instead of a traditional CDN.

Finally I could take advantage of things like [Certianly](ihttps://docs.fastly.com/products/certainly) and never have to worry about updating my TLS certificates again, something that definitely happened to me at least 2 a year while I was trying to get `certbot` to play nice with cron jobs.

## How To Get Started 

There's essentially two paths you have to work on in parallel the raw code for the blog and the setting up of the networking and Fastly Compute to serve said code from their network. 

### Some Warnings First 

I am definitely not done with the blog that's for sure! As well as I am very certain there are better ways to do things but I wanted to give a little insight into how I learned and pieced together the information to get this working. If you have any suggestion throw in [an issue](https://github.com/deg4uss3r/hosfe.lt/issues) for me!

Finally, this blog and post (and my life) will be centered around Rust. There's plenty of resources for other languages (like [JavaScript](https://developer.fastly.com/learning/compute/javascript/) and [Go](https://developer.fastly.com/learning/compute/go/), but I know very little about those).

### The Blog's Code 

The main focus of this section will be my `main.rs` Rust module ([on GitHub](https://github.com/deg4uss3r/hosfe.lt/src/main.rs)). The source HTML is also in there but that is less interesting and I will breifly cover that later. 

The main content is very simple just a large `match` statement based off of the incoming request's path and then serving that in a response through pre-compiled HTML. essentially this boils down to very few lines of code: 

```rust
match req.get_path() {
  "/" => include_str!("../index.html"),
  _ => include_str!("../404.html"),
} 
```

Yes, astute readers will realize that every new page will need a new line to that match statement; however, it's a small price to pay for this being _incredibly simple_. I could setup some glob matches if I hosted the pages on an external service (e.g. S3, Droplet, etc.). That's something I'd like to explore on a future date but for now this is fast, convenient, and fits nicely into a simple enough workflow for a very causual website updater 😅.

A quick note on the `include_str!()` [macro](https://doc.rust-lang.org/std/macro.include_str.html). If you're not immediately familiar this is including the input file in the compiled binary. So while sacraficing binary size by including the text there's no filesystem lookup or network hit required to serve each request. It also has the advantage that if that file doesnot exist the binary will not compile so I cannot serve you a file that does not exist (but I can leave out files that _do_ exist).

### Okay, But How Do You Get Started? 

So that is all simple enough but getting started was a little bit confusing for me (I could not find a specific set of coheisive steps to get started) so I'll walk through how I did it. Previous ceavat applies here, this is information I cobbeled together and not the _best_ way but a way that works. 

First, I created my [Fastly account](https://manage.fastly.com/home) (**disclaimer** this was free for me as a Fastly employee under small usages which this site _is incredibly small_ compared to what else we do). Your experience here will vary please check the prices and understand the cost before you proceed.

Next, is to create a compute service. Do not worry about a domain or host for now you can set it to anything you want and change it later. We will utitlize the test generated link and setup the networking later. Make sure to save your `service_id` as it will be necessary to push an update to that newly created service.

To use the Fastly CLI (next step) you'll need a new token with permissons to do so. In the Fastly Management domain go to your Profile (upper right) > Account > API Tokens (lower left) and generate an API token with Global API Access both the first option (`Global`) for full control and Global Read (`global:read` is enable by default but write is not).

**NOTICE** make _sure_ you save this token off as soon as you navigate away from this screen you will lose access to display the token again for security reasons.

**WARNING** make sure to keep this safe it's a secret and just like an AWS S3 key could cost you a lot of money if you leak it. I suggest immediately storing it in a password manager like [1Password](https://1password.com/) so you can access it safely and from the CLI (I'll show you how to do this as well). Finally, for security I do recommend letting this expire and generating a new one roughly every 6 months.

After that I installed the [Fastly CLI](https://developer.fastly.com/reference/cli/) (or you can do everything from the web if you perfer but I like using CLIs so I do when I can). For me that was as simple as following the `brew install fastly/tap/fastly` command. 

Next up we'll test out a working CLI interface. I use 1Password's CLI to help insert secrets without leaking them (read the install instructions here: [1Password CLI](https://developer.1password.com/docs/cli/get-started/)); however, you can do what you are most comfortable with (both security and tool-wise). The command I would recomment running first is: 

```fish
~# fastly whoami --token $(op item get "$YOUR_FASTLY_ITEM_NAME" --fields $YOUR_API_TOKEN_FIELD_NAME)
```

If you get an output with your name and email you are good to go! Otherwise it would appear the account token you've generated isn't working or you have a previous configuration somehwere that is messing with it. Reach out to the [Fastly Developer Docs](https://developer.fastly.com/) or the [Contact the community](https://community.fastly.com/) for additonal help.

Once you have a succesful result from `whoami` you're ready to push to your service! I would recommend following the [Rust template repository](https://github.com/fastly/compute-starter-kit-rust-default) by forking it and pushing a single simple page (e.g. one match arm with the catchall as well) with anythign you want. Do so by calling the following inside the template repo: 

```fish
~# fastly compute publish --token $YOUR_TOKEN --service-id $YOUR_SERVICE_ID
```

Once that deploys you can check it via the Managment portal, click on Compute > Your Service > Version Number > Then you should see "Test Domain" next to your domain. 

As you get more complex or if you want to test the binary locally you can do so very easily just: 

```fish
~# fastly compute serve
```

That will create the binary and host it locally to `127.0.0.1:1313`.

There's quite a few starter kits on the [Fastly Organization](https://github.com/search?q=%22fastly%2Fcompute-starter-kit%22+owner%3Afastly+&type=repositories) take a look through I just picked the one I found the simplest to get started!

#### Fastly's Template Structure Explained

If you fork one of Fastly's template repositories there's some files in there that probably need a bit of explaining. I'll quickly go over those. 

`rust-toolchain.toml` - You might be already familiar with this but it's essentially controlling what version of Rust and what compile targets you need. This one helps compile for WASM which we need for deployment. From here you can add more options but I just left it as default stable toolchain. 

`fastly.toml` - This is the controlling file for the options on your Compute service. You'll want to change the author, decription, name, and add in the field `service_id` (I originally thought this was a secret but common practice allows for this in the `toml`). Adding in `service_id` now means it will be easier to get the GitHub action working (coming in part 2) as well as you can remove the `--service_id` option from all future command line agruments. 

`Cargo.toml` - I will assume you know the basics here but one thing you'll definitely want to change if you fork is the name (of the package), author, and I also upped the edition from `2018` to `2021` with no ill affects. I would recommend keeping [publish](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field) to `false`.

`README` - I definitely forgot to change this at first, so learn from my mistakes! 

## Fin

That is pretty much all you need to really get started with the edge using a test domain, In the next post (since this one has gotten a little longer than I anticipated) I'll explain my DNS and deployment setup. I hope this was helpful!
