The module adds three Drupal permissions that you can assign per role under `People > Permissions`.

- **Administer FastComments** - Access to the FastComments settings form at `/admin/config/content/fastcomments`.
- **View FastComments** - Required to see the commenting widget. Without this permission the widget does not render.
- **Toggle FastComments** - Allows users to enable or disable comments on a per-entity basis using the field widget.

By default, only users with the `administer site configuration` permission can change FastComments settings. Grant `View FastComments` to anonymous and authenticated users if you want visitors to see the widget.
