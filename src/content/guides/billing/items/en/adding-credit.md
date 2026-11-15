Top-ups are charged to the payment method saved on your account. If you have not saved a card yet, the Credits page shows `Add a payment method first` with a `Go to billing` button. Save a card on the [Billing Info](https://fastcomments.com/auth/my-account/billing-info) page, then come back.

To add credit:

1. Open [My Account -> Billing Info](https://fastcomments.com/auth/my-account/billing-info) and click `Manage Credits`.
2. Under `Add credit`, pick one of the preset amounts, or choose `Other amount` and enter your own.
3. Click the `Add` button. It shows the amount you chose, for example `Add $25.00`.
4. If your bank asks you to confirm the payment, complete the check in the window that opens.

When the payment goes through, the page shows `Credit added` with your new balance, and the top-up appears in `Past top-ups` as `Credited`. Click `Add more` to top up again.

### Amounts and Limits

The presets are `10`, `25`, `50`, and `100` in your billing currency. A custom amount can be anything from `5` to `10,000` per top-up, and the exact limits for your currency are shown under the form. For currencies without decimal places, such as Japanese yen, the presets and limits are one hundred times larger so they represent roughly the same money.

Top-ups are always in your account's billing currency, the same currency as your invoices. The page tells you which currency will be charged.

### Bank Confirmation (3D Secure)

Some banks require you to confirm an online payment, usually through your banking app or a one-time code. If yours does, a confirmation window opens after you click `Add`. Your card is only charged, and your balance only updated, after the bank approves.

If you close the page before finishing the confirmation, the top-up is recorded as `Pending`. The next time you open the Credits page we check the payment's status with Stripe and mark it `Credited` or `Failed` accordingly. If the bank does not approve the payment, the page shows `Your bank did not approve the payment. Try again, or use a different card.` and nothing is charged.

### Declined Cards

If your card is declined, the page shows `Your card was declined. Check your card on the billing page, or try another card.` Nothing is charged, your balance does not change, and the attempt is listed in `Past top-ups` as `Failed`. Update the card on the [Billing Info](https://fastcomments.com/auth/my-account/billing-info) page and try again.

To protect your card, an account can make a limited number of top-up attempts per hour. If you see `Too many top-up attempts. Wait a while and try again.`, wait and retry later.

Payments are processed by Stripe. FastComments never stores your card details.
