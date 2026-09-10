## Actions and Searches

Actions create data in FastComments; searches look data up so a later step can use it. Each action calls
the FastComments REST API and spends the same API credits the call would cost from your own code: one
credit per call unless noted.

## Create Comment

Posts a comment on a page.

| Field | Required | Notes |
|-------|----------|-------|
| Page URL ID | Yes | The URL ID the comment widget uses on the page. Comments are grouped by it. |
| Page URL | Yes | The full page URL, used in notification emails. |
| Comment | Yes | The comment body in FastComments markdown. |
| Commenter Name | Yes | Names are unique per email, so reusing a name with a different email fails. |
| Commenter Email | No | A user is created for the email when it does not exist yet. |
| User ID | No | An existing SSO user id. Takes precedence over the name and email. |
| Parent Comment ID | No | Set to post a reply. |
| Approved, Verified | No | Both default to true. Unapproved comments stay hidden until moderated. |
| Posted At | No | Defaults to now. |
| Avatar URL, Page Title, Locale | No | Locale defaults to `en_us`. |
| Show Live In Widget | No | Pushes the comment to viewers in real time. Costs 2 credits instead of 1. |
| Run Spam Check, Send Emails | No | Off by default. |

## Create Page

Creates a page record before any comment exists on it, so it can be listed and restricted. Takes the URL ID,
title, URL, and optionally the SSO group ids allowed to see it.

## Create SSO User

Creates a single sign-on user. Takes your own user id, username and email, plus optional display name,
display label, avatar, website, group ids, and notification and privacy flags. Administrative roles cannot
be granted from Zapier.

## Create Feed Post

Creates a post in a FastComments feed from HTML content, with an optional title, author, tags, and one link
preview.

## Create Hash Tag

Creates a hash tag that commenters can use, with an optional URL it links to.

## Flag Comment

Flags a comment for moderator review. Provide the id of the user doing the flagging, or leave it blank to
flag as the Zapier integration.

## Searches

| Search | Input | Returns |
|--------|-------|---------|
| Find Comment | Comment ID | The comment, or nothing. |
| Find SSO User | Email | The SSO user, or nothing. |
| Find Page | URL ID | The page, or nothing. |

A search that finds nothing does not fail the Zap. Combine a search with a create in Zapier's
"find or create" mode to create the page or user when it is missing.
