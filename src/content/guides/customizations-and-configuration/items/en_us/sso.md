SSO, or single-sign-on, is a set of conventions used to allow you or your users use FastComments without having to create another account.

Assuming you don't allow anonymous commenting, an account is required to comment with FastComments. We make this sign-up process very easy - the user just leaves their email when they comment.
However, we understand that even that is extra friction some sites want to avoid.

We can reduce that friction by only having one login flow for your entire site.

### How do I get it?
All account types currently get access to SSO. However, the maximum number of SSO users will vary depending on your package. As with other features, the Pro plans and higher provide direct development support.

Let's compare the options, and then go into details of each.

### User and Comment Migrations

When migrating from a platform with SSO like Disqus, you will already have users and their comments.

Comments are imported as part of your migration, either by the API, our Import UI, or customer support. The Import UI is preferred if it supports the platform you
are migrating from, as it incorporates error handling, avatar and media extraction and uploads, and a batch job monitoring system.

The users themselves are added automatically when viewing comment threads for the first time. Alternatively, they can be pre-added via the API, but this work does not have many advantages.

If comments are imported, and SSO users are not added manually via the API, then comments will automatically be migrated to the user's account the first
time it is created when they view any comment thread. They will then be able to manage, edit, and delete the comments they originally wrote.

The automatic migration is done via email or username. Some platforms do not provide emails on export, like Disqus, so we fall back to username in this case.
- As long as you pass a matching username, and an email in the SSO payload, we will add the email to the individual comment objects so that notifications and mentions work.

If it is desired to import your comments and users at once, work with support to migrate the comments over to the users' respective accounts after users are imported
via the API.

So to summarize the easiest path to take for the migration is:

1. Import comments.
   1. Avatars and other media are migrated automatically if using the Import UI in `Manage Data -> Imports`.
2. Set up Secure or Simple SSO.
3. Let the migration happen per-user automatically when they log in for the first time.
   1. This usually adds less than a second to the page load time if the user has fewer than 50k comments.

### WordPress Users
If you're using our <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress plugin</a> then there is no code to write! Simply go to the plugin's Admin page, click SSO Settings, and then Enable.

This will take you to a single-button click wizard which will create your API key, send it over to your WordPress install and turn SSO on. We've consolidated this into a single button click for you.

Note that if you are installing the plugin for the first time you will have to follow up the setup process before you see the admin page with the SSO Settings button.

#### WordPress SSO - Moderators

Note that currently for the "Moderator" badge to show next to your moderators when they comment with the FastComments WordPress plugin,
they must also be added as a Moderator in the FastComments dashboard and have their email verified.

### Custom Integrations

For Custom integrations, there are two options.

### Option One - Secure SSO

With Secure SSO, FastComments knows that the user commenting, voting, and reading comments is a real user on your site.

As long as you create a valid payload, the user will always have a seamless commenting experience.

With Secure SSO, the SSO payload is created **server-side** using HMAC authentication and then passed to the widget on the **client**.

With Secure SSO, the user's account is **completely separate** from the rest of the FastComments user-base. This means if we have two partners
Company A and Company B, each can have an SSO user with the username "Bob".

#### Requirements
- Some basic knowledge of backend development.
- Some basic knowledge of dealing with secret API keys.
- Some basic knowledge of API development or server-side rendering.

#### Pros
- Secure.
- Seamless commenting experience.

#### Cons
- Requires backend development.

#### Updating User Data

With Secure SSO, each time you pass the sso user payload, we will update their user with the latest information. For example, if
the user has a username `X`, and you pass `Y` in the SSO payload, their username will become `Y`.

If you want to remove values using this approach then set them to `null` (not `undefined`).

#### Secure SSO API

We also provide an API for interacting with the SSO users. See [the docs](/guide-api.html#sso-user-structure).

Note that when using Secure SSO, users are automatically created behind the scenes on page load. You do not have to bulk import your users.

### Option Two - Simple SSO

The alternative to Secure SSO is to simply pass the user information to the commenting widget.

Providing an email with Simple SSO is not required, however without this their comments will show as "Unverified".

<sup>Note!</sup> As of early 2022 usernames with Simple SSO do not need to be unique across all of FastComments.com.

Ideally, Simple SSO should only be chosen when developing on a platform that doesn't provide backend access.

#### Requirements
- Some basic knowledge of client-side development.
- You need to know at least the user's email.

#### Pros
- Simple.
- All activity still gets verified.
- The user never enters their username or email.

#### Cons
- Less secure than Secure SSO as the client-side payload could be crafted to impersonate any user.

#### Simple SSO API

Users automatically created via the Simple SSO flow are stored as `SSOUser` objects. They can be accessed and managed via the `SSOUser` API. See [the docs](/guide-api.html#sso-user-structure).