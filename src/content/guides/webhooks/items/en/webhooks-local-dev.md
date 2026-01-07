For Local development, use a tool like [ngrok](https://ngrok.com/).

In order to simplify keeping the system secure, local development follows the same process as setting up and securing other environments. 

### Step 1: Add "localhost" to domains in your account.

Add "localhost" [as a domain here](https://fastcomments.com/auth/my-account/configure-domains).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/configure-domains/new'; selector = '.content .account-block'; title='Add localhost'; actions=[{type: 'set-value', selector: 'input[name=domainNames]', value: 'localhost'}] app-screenshot-end]

### Step 2: Pick an API Key

We're going to be adding webhook configuration for your domain, so we'll need an API key. [You can do that here.](https://fastcomments.com/auth/my-account/api-secret)

[app-screenshot-start url='https://fastcomments.com/auth/my-account/api-secret/add'; selector = '.content .account-block'; title='Add Testing API Key'; actions=[{type: 'set-value', selector: 'select[name=domain]', value: 'localhost'}, {type: 'set-value', selector: 'input[name=name]', value: 'Testing'}] app-screenshot-end]

Under "Associate with domain" - select your "localhost" domain.

**NOTE: Alternatively, you can use one API Secret for all testing activity and staging environments. Simply add an API Secret for "All Domains", and give it a name like "test".**

Ensure you have an API Secret defined for your production domain(s). Events for all other domains will use the wildcard (testing) secret.

### Step 3: Add Your Webhook

While running ngrok or similar tool, set the value for "localhost" [here](https://fastcomments.com/auth/my-account/manage-data/webhooks).

[app-screenshot-start url='https://fastcomments.com/auth/my-account/manage-data/webhooks'; selector = '.content'; title='Add Testing Webhook'; actions=[{type: 'wait', selector: 'button[type=submit]'}, {type: 'set-value', selector: '#domain-select', value: 'localhost'}, {type: 'set-value', selector: 'input[name="comment-created-url"]', value: 'http://xxxx-xxxx-xxxx-xxxx.ngrok.io/some-route'}] app-screenshot-end]

When clicking `Send Test Payload`, we will send two test events to check that you validate the API key.

Once it validates, hit `Save`.

### Step 4: Add A Comment

Now you can add, edit, or delete comments and should see us call your local development machine with the events, using your testing API key. There may be up to 30 seconds delay
for the events to reach your machine.
