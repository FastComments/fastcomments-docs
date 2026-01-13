Each instance of the comment widget is isolated. Because of this, FastComments inherently supports more than one instance per page, or multiple
instances pointing to the same chat thread.

In the case of the VanillaJS library for example, you simply have to tie the comment widget to different DOM nodes. If you want to simply
update the current thread on the page, see [Switching Comment Threads Without Reloading The Page](guide-customizations-and-configuration.html#switching-comment-threads);

### Syncing Authentication State Across Multiple Instances

Let's go over the example of a custom single-page-application that is a list of frequently asked questions with their own comment thread.

In this case, we have multiple instances of FastComments in the DOM at once.

This is fine, but it poses some challenges for user experience.

Consider this flow:

1. The user visits the page with a list of questions, each with their own comment widget.
2. The user enters their username and email and leaves a question on one of the threads.
3. They see another FAQ item they have a question about.
4. They go to comment again. Do they have to enter their email and username again?

In this case, FastComments handles syncing the authentication state across widget instances for you. In step four, the user
will already be temporarily authenticated since they entered their username and email on the same page.
