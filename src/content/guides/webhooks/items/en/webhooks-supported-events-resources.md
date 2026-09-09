FastComments supports webhooks for the Comment resource only.

We support webhooks for comment creation, removal, and on update.

Each of these are considered separate events in our system and as such have different semantics
and structures for the webhook events.

Any number of endpoints can subscribe to the same event: one webhook per domain can be configured in
the dashboard, and further subscriptions can be created through the API (see Managing Webhooks via the API).
