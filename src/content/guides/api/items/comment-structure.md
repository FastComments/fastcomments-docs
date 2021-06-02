A `Comment` object represents a comment left by a user.

The relationship between parent and child comments is defined via `parentId`;

The structure for the Comment object is as follows:

[inline-code-attrs-start title = 'Comment Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    id: string;
    /** The user that wrote the comment. Created automatically when saving a comment with a name/email. **/
    userId?: string|null;
    /** A "cleaned" version of the urlId you passed us. When saving, you specify this field, but when you fetch the comment back this will be "cleaned" and your original value moved to "urlIdRaw". **/
    urlId: string;
    /** READONLY: The original urlId you passed us. **/
    urlIdRaw?: string;
    /** The URL to the location that this comment is visible, like a blog post. **/
    url: string;
    /** The title of the page this comment was on. **/
    pageTitle?: string;
    /** The commenter's name. Always required. If not available, set to something like "Anonymous". **/
    commenterName: string;
    /** The commenter's email. Required if anonymous commenting is off. **/
    commenterEmail?: string;
    /** The commenter's link (for example, their blog). **/
    commenterLink?: string;
    /** The commenter's raw comment. **/
    comment: string;
    /** The locale the comment is in. If not provided, will be derived from the language accept HTTP header. **/
    locale?: 'en_us' | 'es_es' | 'fr_fr' | 'pl_pl' | 'de_de' | 'it_it' | 'ru_ru';
    /** READONLY: The commenter's comment parsed into HTML. **/
    commentHTML?: string;
    /** If we're replying to a comment, this is the ID that we are replying to. **/
    parentId?: string|null;
    /** The date the comment was left, in UTC epoch. **/
    date: number;
    /** The "karma" of the comment (= votes up + votes down). **/
    votes?: number;
    /** Is the user and this comment verified? **/
    verified: boolean;
    /** The user's avatar. **/
    avatarSrc?: string;
    /** READONLY: Does the comment contain images? **/
    hasImages?: boolean;
    /** READONLY: Does the comment contain links? **/
    hasLinks?: boolean;
    /** READONLY: Is the comment by an admin? **/
    isByAdmin?: boolean;
    /** READONLY: Is the comment by a moderator? **/
    isByModerator?: boolean;
    /** Is the comment pinned? **/
    isPinned?: boolean;
    /** The "display label" for the comment - for example "Admin", "Moderator", or something like "VIP User". **/
    displayLabel?: string;
    /** A star rating. **/
    rating?: number;
    /** Whether or not notifications were sent for this comment for commenters. To prevent notifications being sent on imports, set this to true. **/
    notificationSentForParent?: boolean;
    /** Whether or not notifications were sent for this comment for tenant users. To prevent notifications being sent on imports, set this to true. **/
    notificationSentForParentTenant?: boolean;
}
[inline-code-end]

Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.
