FastComments supports webhooks for the Comment resource only.

We support webhooks for comment creation, removal, and on update.

Each of these are considered separate events in our system and as such have different semantics
and structures for the webhook events.

Any number of endpoints can subscribe to the same event, from the dashboard or through the API
(see Managing Webhooks via the API). Each webhook is delivered independently.
