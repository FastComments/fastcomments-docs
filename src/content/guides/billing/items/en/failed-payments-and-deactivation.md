A payment can fail because a card expired, was replaced, has no funds, or the bank blocked the charge. FastComments gives you time to fix it before anything stops working, and tells the people who can fix it.

### What Happens

1. **The charge fails.** The invoice stays `Unpaid` in [Billing History](https://fastcomments.com/auth/my-account/billing-history) with a `Pay Now` button. On a fixed plan Stripe may retry the charge. On Flex, FastComments retries any unpaid invoice older than the grace period every night.
2. **A warning is sent.** Each night FastComments checks every account. If a Flex invoice is still unpaid past the grace period and the retry fails, or a fixed-plan subscription has ended because it could not be collected, every user with the `Billing Admin` permission gets the email `Your FastComments Tenant Will Be Deactivated Soon!` with the date, and the account is scheduled for deactivation one grace period later. The widget keeps working in the meantime.
3. **Deactivation.** If nothing has changed by the scheduled date, the account is deactivated: comments are hidden on your site and visitors see `This FastComments account's billing information is more than sixty days out of date. Comments hidden until the issue is resolved.` Billing admins are emailed again. Your data is kept.

The grace period is one week for most accounts and two weeks for accounts older than a year. On Flex it applies twice, once before the warning and once between the warning and deactivation, so there are at least two weeks between a failed charge and comments being hidden.

### Fixing It

Save a working card on the [Billing Info](https://fastcomments.com/auth/my-account/billing-info) page. That alone reactivates the account immediately and cancels any scheduled deactivation; the unpaid invoice is then collected with the new card, immediately on a fixed plan and during the nightly check on Flex. You can also pay the invoice straight away with `Pay Now` in Billing History.

If the card on file is fine and the failure was temporary, you do not have to do anything: the nightly check notices the paid invoice or active subscription, cancels the deactivation, and emails the billing admins to confirm. An already deactivated account is reactivated the same way.

### Several Invoices At Once

If your card was invalid for a few months and then fixed, each unpaid month is collected separately, so you may see several charges on the same day. Check the billing period on each invoice in Billing History: they are for different months, not duplicates.

### Still Seeing A Billing Warning After Paying?

The warning belongs to a specific account. If you paid but a site still shows `billing information is out of date`, the widget on that site is most likely using a different account than the one you paid on. This happens when a site was set up twice, or when there is one account on `fastcomments.com` and another on the EU deployment at `eu.fastcomments.com`, which are billed separately. Compare the `tenantId` in the code on your page with the one under `Settings -> API/SSO`, or [contact support](https://fastcomments.com/auth/my-account/help) and we will straighten it out.
