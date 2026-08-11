Follow the same steps for `localhost` as you would production. Ensure you have production domains and API Secrets setup.

First, navigate to the [διαχειριστικό Webhooks](https://fastcomments.com/auth/my-account/manage-data/webhooks). This is accessible via Διαχείριση Δεδομένων -> Webhooks.

The configuration page appears as follows:

[app-screenshot-start url='/auth/my-account/manage-data/webhooks'; selector = '.content'; alt='Σελίδα διαχείρισης Webhooks με επιλογέα τομέα και πεδίο URL τελικού σημείου ανά γεγονός σχολίου, συν τη λειτουργία Αποστολή Δοκιμαστικού Φορτίου'; title='Διαμόρφωση Webhooks'; cacheBuster = 'v3' app-screenshot-end]

In this page you can specify endpoints for each type of comment event.

For each type of event, be sure to click Send Test Payload to ensure you've set up your integration correctly. See the next section, "Testing", for details.