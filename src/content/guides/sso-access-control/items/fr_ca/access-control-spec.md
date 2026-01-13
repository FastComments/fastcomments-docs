Définir la façon dont plusieurs utilisateurs interagissent, et le tester, est compliqué. Voici la spécification suivante que nous suivons pour le contrôle d'accès,
que vous pouvez utiliser pour tester votre implémentation :

    Page avec null group ids, utilisateur avec null group ids - devrait avoir accès.
    Page avec null group ids, utilisateur avec group ids - devrait avoir accès.
    Page avec group ids, utilisateur avec null group ids - devrait avoir accès.
    Page avec group ids, utilisateur avec liste vide - ne devrait PAS avoir accès.
    Page avec group ids, utilisateur avec group ids - l'intersection existe - devrait avoir accès.
    Page avec group ids, utilisateur avec group ids - l'intersection n'existe pas - ne devrait PAS avoir accès.
    Page avec liste vide de group ids (personne n'a accès), utilisateur avec null - ne devrait PAS avoir accès.
    
    SSO User A = Aucun group ids défini (null = accès complet).
    SSO User B = Aucun group ids défini (null = accès complet).
    A peut @B.
    
    SSO User A = Aucun group ids défini (null = accès complet).
    SSO User B = Group ids définis.
    A peut @B.
    
    SSO User A = Group ids définis.
    SSO User B = Aucun group ids défini (null = accès complet).
    A peut @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [b].
    A ne peut PAS @B.
    
    SSO User A = Group ids = [a].
    SSO User B = Group ids = [a, b].
    A peut @B.