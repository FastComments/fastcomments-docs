The FastComments API exposes the following resources to you:

- Comments
- Pages
- Votes

Currently, you can only fetch these resources. You cannot leave comments via the API.

### Resource Usage

It should be noted that fetching data from the API is counted as usage on your account.

Each resource will list what that usage is in its own section.

If a resource says "1 Page Load: = 1000 Votes Max", this means that a request that returns 1000 votes will count as
one `page load` (or credit) on your account. Your account has a limited number of these based on its tier (the same as
page loads). If the API returns 1,500 Vote objects, this would count as two page loads on your account. If you make two API calls, and each
return 10 Votes, that also counts as two page loads as they are separate requests.

#### Note!

We have listed this documentation in non-alphabetical order, listing the Pages documentation first, to help
limit confusion when determining what values to pass for `urlId` in the Comment API.
