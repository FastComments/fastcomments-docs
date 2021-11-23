A `HashTag` object represents a tag that can be left by a user. HashTags can be used to link to an external piece of content or to
tie related comments together.

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'HashTag Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Should start with the "#" or desired character. **/
    tag: string;
    /** An optional URL that the hashtag can point to. Instead of filtering comments by hashtag, the UI will redirect to this upon click. **/
    url?: string;
    /** READONLY **/
    createdAt: string;
}
[inline-code-end]

Notes:

- In some of the API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.
 
