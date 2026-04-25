---
대부분의 사이트에서 댓글을 추가하는 가장 쉬운 방법은 `FastComments` 필드를 콘텐츠 유형에 추가하는 것입니다. `Structure > Content types > [type] > Manage fields`로 이동하여 필드를 추가하세요.

Each entity that has the field gets:

- 편집자가 각 엔티티에서 댓글 기능을 켜거나 끌 수 있는 **상태 토글**.
- 선택적 **사용자 지정 식별자**로, Drupal 엔티티 경로에 묶여 있지 않은 안정적인 ID를 사용할 수 있습니다.

The main `FastComments Widget` block knows about this field, and will skip entities that already have it attached. That way you can mix per-entity comments with the block without seeing the widget twice on the same page.
---