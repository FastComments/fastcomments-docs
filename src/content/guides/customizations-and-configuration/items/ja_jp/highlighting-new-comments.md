FastComments は新しいコメントを強調表示するためのいくつかの方法を提供します。

First and foremost, by default comments that triggered an in-app notification (replies, replies in same thread, or comments on a page
あなたが購読しているページのコメント）が自動的にユーザーのアバターがわずかに光る形で強調表示されます。色は CSS
を使用して `is-unread` クラスでカスタマイズできます。

過去24時間に投稿されたコメントには `24hr` クラスが適用されており、スタイリングに使用できます。

Finally, any new live comments that show up in the user's session will be highlighted for several seconds via an animation. This is done via the
`is-live` CSS class and can be customized as well.