A `Comment` object represents a comment left by a user.

The relationship between parent and child comments is defined via `parentId`;

The structure for the Comment object is as follows:

[inline-code-attrs-start title = 'Comment Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    id: string;
    tenantId: string;
    userId?: string|null;
    urlId: string;
    commenterName: string;
    commenterLink: string;
    commentHTML: string;
    parentId?: string|null;
    date: string;
    votes: number;
    verified: boolean;
    avatarSrc: string;
    hasImages: boolean;
    hasLinks: boolean;
    isByAdmin?: boolean;
    isByModerator?: boolean;
    isPinned?: boolean;
    displayLabel?: string;
    rating?: number;
}
[inline-code-end]
