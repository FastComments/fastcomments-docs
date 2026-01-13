A major benefit of SSR is that JavaScript is not required. Because of this, applications can be built to feel "lighter" in many use cases.

Additionally, SSR can be used as a fallback in the event that the user has JavaScript disabled. This way comment threads can still render, and the user
can still reply to comments.

FastComments is already well optimized, so in most cases SSR is not required. However, some online communities have users which do not use JavaScript, or disabling
it is the preferred practice. This is where FastComments SSR can be useful.

However, a major trade-off of SSR is having to reload the page for small state changes.