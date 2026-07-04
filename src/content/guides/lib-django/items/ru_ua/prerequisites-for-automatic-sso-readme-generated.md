To pass the logged-in user to the widget automatically, the tags read the
current user from the request. Make sure your project has both of these (they
are on by default in a standard Django project):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Without a request in the template context, widgets render for an anonymous
visitor. You can always pass a user explicitly: `{% fastcomments user=some_user %}`.