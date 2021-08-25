SSO, or single-sign-on, is a set of conventions used to allow you or your users use FastComments without having to create another account.

Assuming you don't allow anonymous commenting, an account is required to comment with FastComments. We make this sign up process very easy - the user just leaves their email when they comment.
However, we understand that even that is extra friction some sites want to avoid.

We can reduce that friction by only having one login flow for your entire site.

### How do I get it?
All account types - from the $4.99/mo plan to the $350/mo plan - get access to SSO. As with other features, the Pro plan provides direct development support.

Let's compare the options, and then go into details of each.

### WordPress Users
If you're using our <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">WordPress plugin</a> then there is no code to write! Simply go to the plugin's Admin page, click SSO Settings, and then Enable.

This will take you to a single-button click wizard which will create your API key, send it over to your WordPress install and turn SSO on. We've consolidated this into a single button click for you.

Note that if you are installing the plugin for the first time you will have to follow up the setup process before you see the admin page with the SSO Settings button.

#### WordPress SSO - Moderators

Note that currently for the "Moderator" badge to show next to your moderators when they comment with the FastComments WordPress plugin,
they must also be added as a Moderator in the FastComments dashboard, and have their email verified.

### Custom Integrations

For Custom integrations, there are two options.

### Option One - Secure SSO

With Secure SSO, FastComments knows that the user commenting, voting, and reading comments is a real user on your site.

As long as you create a valid payload, the user will always have a seamless commenting experience.

With Secure SSO, the SSO payload is created **server-side** using HMAC authentication and then passed to the widget on the **client**.

With Secure SSO, the user's account is **completely separate** from the rest of the FastComments user-base. This means if we have two partners
Company A and Company B, each can have an SSO user with the username "Bob". This is not possible with **Simple** SSO if emails are provided.

#### Requirements
- Some basic knowledge around backend development.
- Some basic knowledge around dealing with secret API keys.
- Some basic knowledge around API development or server-side rendering.

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

We also provide an API for interacting with the SSO users. See [the docs](/guide-api#sso-user-structure).

### Option Two - Simple SSO

The alternative to Secure SSO is to simply pass the user information to the commenting widget. They won't actually be logged in, but it
will appear as so in the commenting widget. The reason for this is we can't actually create a "session" - however instead we will
email them when they take action like commenting or voting to verify their activity, without ever asking them for their username or email.

Note that providing an email with Simple SSO is not required, however by default their comments will show as "Unverified".

However, if invalid information is provided, for example: An email is provided along with a username, Bob. However, there is already a FastComments
user with the user Bob. So, when the user goes to comment, they would get an error to pick a different username. This is one of the trade-offs of using Simple SSO.

This is due to the fact that when providing an email, we seamlessly create an account for the user. However, if only a username is provided, we will not.
Please note that the user will be sent a welcome email to verify their account. If they wish, they can also delete their account and keep their comment.

Ideally, Simple SSO should only be picked when developing on a platform that doesn't provide backend access.

#### Requirements
- Some basic knowledge around client-side development.
- Have to know at least the user's email.

#### Pros
- Simple.
- All activity still gets verified.
- The user never enters their username or email.

#### Cons
- Username must be unique when paired with an email.
- If Username is not unique, the user will go through the same flow as if not using SSO.
- The user now has to deal with another account, not only the account associated with your website.

#### Simple SSO API

We do not provide an api for interacting with the users created from the Simple SSO flow.
