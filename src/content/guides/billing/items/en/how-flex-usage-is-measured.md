A Flex invoice is a list of line items, one per kind of usage, each showing the quantity and the rate, for example `213,000 Page Loads. ($1.00 per 100,000 Page Loads)`. The rates are on the [pricing page](https://fastcomments.com/traffic-pricing) and on your [Billing Info](https://fastcomments.com/auth/my-account/billing-info) page. This section explains what each quantity means.

### Counted Over The Month

These reset every month and are counted from the first to the last day of the calendar month (UTC):

- **Page Loads**: the number of times a comment widget or feed loaded on your pages. Every load of the comment widget counts, no matter how many comments it shows. There is no charge per comment thread or per page.
- **API Credits**: calls to the REST API with your API key.
- **Small Widgets Credits**: loads of the small widgets, such as comment counts and badges.
- **LLM Tokens**: tokens used by [AI Agents](/guide-ai-agents.html) and other LLM features.

### Counted At The End Of The Month

These are a snapshot taken when the invoice is created:

- **Comments Stored**: the total number of comments in your account, not just the ones added this month. Deleting comments reduces it, except that a deleted comment with replies is kept as a placeholder so the replies still make sense.
- **SSO Users**: the total number of SSO users your site has created, including people who commented once a long time ago. If this is the biggest part of your bill, ask support about billing by monthly active users instead, where only users who logged in during the month count.
- **Moderators**, **Admins**, **SSO Moderators**, and **SSO Admins**: seats, covered in [Admin and Moderator Seats](#admin-and-moderator-seats). The first admin is free.
- **Domains**: the sites in your account. The first site is free and each additional one is billed per month, for as long as it is in the account. A subdomain only counts if you add it as its own site; a single top-level domain covers its subdomains. See [Multiple Sites](/guide-multiple-sites.html#domain-limits).
- **Managed Tenants**: sub-accounts managed by this account, if any, plus their own usage as separate lines.

### Minimum

If the lines add up to less than the Flex minimum, the invoice is topped up to the minimum. A small site with one admin and one domain pays only the minimum.

### Previewing The Invoice

The [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) page shows every line of the next invoice with today's numbers, so there are no surprises. The page load, API, and small widget counts can be up to a minute behind; the rest is live.

### Other Currencies

Flex usage is priced in US dollars. If your invoice currency is something else, each line is converted at the exchange rate on the day the invoice is created, so the same usage can cost slightly different amounts from month to month. See [Currencies](#currencies).
