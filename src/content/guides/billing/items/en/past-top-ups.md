The `Past top-ups` table at the bottom of the Credits page lists your most recent top-ups, newest first, with the `Date`, `Amount`, and `Status` of each one. Before your first top-up it reads `No top-ups yet.`

Each top-up has one of three statuses:

- `Credited`: the payment succeeded and the amount was added to your balance.
- `Pending`: the payment was started but has not finished, usually because a bank confirmation was not completed. Pending top-ups are checked against Stripe the next time the Credits page is opened and move to `Credited` or `Failed`.
- `Failed`: the card was declined or the bank did not approve the payment. Nothing was charged and your balance did not change.

The table records top-ups made from this page. Invoices paid from credit are shown with the invoices themselves, under `View Invoices` on the [Billing Info](https://fastcomments.com/auth/my-account/billing-info) page.
