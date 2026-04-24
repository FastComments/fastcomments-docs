For most sites, the easiest way to add comments is to attach the `FastComments` field to your content types. Go to `Structure > Content types > [type] > Manage fields` and add the field.

Each entity that has the field gets:

- A **status toggle** so editors can turn commenting on or off per entity.
- An optional **custom identifier** so you can use a stable ID that isn't tied to the Drupal entity path.

The main `FastComments Widget` block knows about this field, and will skip entities that already have it attached. That way you can mix per-entity comments with the block without seeing the widget twice on the same page.
