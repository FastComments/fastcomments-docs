The module ships several blocks you can place from `Structure > Block layout` (`/admin/structure/block`).

- **FastComments Widget** - The main commenting widget. Auto-detects the current entity. It will skip entities that already have the FastComments field attached, so you won't see duplicate widgets on the same page.
- **FastComments Live Chat** - Real-time streaming chat. Can be placed alongside the comment field on the same page.
- **FastComments Collab Chat** - Text-selection annotation and discussion.
- **FastComments Image Chat** - Coordinate-based annotation on images. Visitors click on an image to leave comments tied to specific locations.
- **FastComments Recent Comments** - Displays recent comments across your site. The count is configurable on the block.
- **FastComments Top Pages** - Shows the pages on your site with the most comments.

The content-centric blocks (Live Chat, Collab Chat, Image Chat) auto-detect the current entity, and fall back to a path-based identifier on non-entity pages. That means they work on taxonomy pages, views, and custom routes without any extra setup.
