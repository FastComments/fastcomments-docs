What your card is charged, and when, depends on the kind of plan you are on.

### Fixed Plans

When you start a `Creators` or `Pro` subscription, the first month or year is charged immediately, in the browser, when you click `Pay Now & Start Subscription`. If your bank asks you to confirm the payment, complete the check in the window that opens. The subscription only starts once the payment has gone through.

After that, the subscription renews on the same day of each month, or of each year if you chose `Pay Annually`, counted from the day you subscribed. The renewal is charged to the card on file automatically and the invoice is emailed to your billing email. There is no action to take.

Yearly billing is 15% cheaper than monthly. Flex cannot be billed yearly.

### Flex

Flex is billed in arrears: you use the service during a calendar month, and the invoice for that month is created on the last day of the month (UTC) and charged to your card right away. The billing date is always the last day of the month and cannot be changed.

The invoice covers that calendar month only. Usage in the calendar month you signed up in is never billed, so your first invoice is at the earliest at the end of the month after you signed up.

If your usage comes to less than the Flex minimum, a line reading `FastComments Flex. (minimum ... /mo)` tops the invoice up to the minimum amount. The current minimum is shown on your [Billing Info](https://fastcomments.com/auth/my-account/billing-info) page and the [pricing page](https://fastcomments.com/traffic-pricing).

You can see what the next invoice will be at any time. The `Next Invoice Amount` is on the [Billing Info](https://fastcomments.com/auth/my-account/billing-info) page, and the full line-by-line breakdown is on the [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) page. What is counted is covered in [How Flex Usage Is Measured](#how-flex-usage-is-measured).

### Saving A Card Does Not Charge It

Once your billing is set up, saving or updating a card without changing plans never charges it, unless you have an unpaid invoice. The form says `You won't be charged now unless you change plans.` and the card is simply stored with Stripe for the next invoice. Only when you have no valid billing yet does the form say `Your card will be charged now.`, and even then a Flex account is not charged until the end of the month. Any prepaid [credit](#credits) on your account is applied to each invoice before the card is charged.

### Managed Sub-Accounts

If your account manages sub-accounts (for example one per site or environment, or white-labeled tenants), only the parent account is billed. Each sub-account's usage appears as its own lines on the parent's Flex invoice, prefixed with the sub-account's name, and sub-accounts share the parent's payment method and billing status.
