By default, FastComments has a very easy and seamless sign up flow. However, we can improve upon that further by only having
one login flow for your entire site. FastComments accomplishes this with SSO, and there are two options.

First, we'll compare the options, and then go into details of each.

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
