Obiekt `Moderator` reprezentuje konfigurację moderatora.

Istnieją trzy typy moderatorów:

1. Użytkownicy administratorzy, którzy mają flagę `isCommentModeratorAdmin`.
2. Użytkownicy SSO z flagą `isCommentModeratorAdmin`.
3. Zwykli komentatorzy, lub użytkownicy FastComments.com, którzy są zapraszani jako moderatorzy.

Struktura `Moderator` jest używana do reprezentowania stanu moderacji w przypadku użycia `3`.

Jeśli chcesz zaprosić użytkownika na moderatora za pomocą API, użyj API `Moderator`, tworząc `Moderator` i `inviting` ich.

Jeśli użytkownik nie ma konta na FastComments.com, wiadomość e-mail z zaproszeniem pomoże mu się skonfigurować. Jeśli ma już konto, otrzyma dostęp moderatorski do Twojego tenanta, a pole `userId` obiektu `Moderator` zostanie zaktualizowane, aby wskazywać na jego użytkownika. Nie będziesz mieć dostępu API do ich użytkownika, ponieważ w tym przypadku konto należy do nich i jest zarządzane przez FastComments.com.

Jeśli potrzebujesz pełnego zarządzania kontem użytkownika, zalecamy albo użycie SSO, albo dodanie go jako [Użytkownik tenanta](https://fastcomments.com/auth/my-account/users) i następnie dodanie obiektu `Moderator` do śledzenia ich statystyk.

Struktura `Moderator` może być używana jako mechanizm śledzenia statystyk dla przypadków użycia `1` i `2`. Po utworzeniu użytkownika dodaj obiekt `Moderator` z określonym polem `userId`, a jego statystyki będą śledzone na [stronie moderatorów komentarzy](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Struktura obiektu `Moderator` jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]

---