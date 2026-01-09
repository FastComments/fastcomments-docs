Since email templates support variables and logic, it is possible to create templates
that fail to render, or that sometimes render inconsistently.

This can be very frustrating to diagnose and debug, especially if it is an intermittent issue, or
if it only occurs when the data looks a certain way.

To help, FastComments Email Templates has a couple of features:

1. If the template fails to preview, it cannot be saved. An error message will be shown.
2. Template render failures are tracked and reported on in the admin UI.

The second bullet is describing render failures that happen in production. As in, you create a template which previews
fine - but later fails for some reason. For example, if we have this in our template:

    <% if (comment.commenterEmail.includes('test') { %>

This may fail sometimes if we have anonymous commenting enabled, since email will not always
be available. So how do we find out about that?

The answer is that errors are surfaced in two places. First, the template list itself
shows a render error count with each template.

Then, when viewing a template we can see a count, per-error, of the number times the template
has failed to render.

A reset button is located next to each error, and its count, so that we can reset the counter
after we have resolved an issue.