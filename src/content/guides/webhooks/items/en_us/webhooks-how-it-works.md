All changes to the Comment object in the system trigger an event that ends up on a queue.

The initial webhook event is usually sent within six seconds of the event source occurring.

You can monitor this queue in the Webhooks admin in the event that your API goes down.

If a request to your API fails, we'll requeue it on a schedule.

That schedule is `1 Minute * the retry count`. If the call fails once, it'll try again in
a minute. If it fails twice, it'll then wait two minutes, and so on. This is so that we
don't overload your API if you're going down for load-related reasons.

Webhooks can be canceled from the [logs page](https://fastcomments.com/auth/my-account/manage-data/webhooks/logs).