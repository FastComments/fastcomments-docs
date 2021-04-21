All changes to the Comment object in the system fire an event which ends up on a queue.

You can monitor this queue in the Webhooks admin in the event that your API goes down.

If a request to your API fails, we'll re-queue it on a schedule.

That schedule is 1 Minute * the retry count. If the call fails once, it'll try again in
a minute. If it fails twice, it'll then wait two minutes, and so on. This is so that we
don't overload your API if you are going down to load related reasons.
