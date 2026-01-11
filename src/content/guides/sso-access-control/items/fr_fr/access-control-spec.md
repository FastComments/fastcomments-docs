Définir la façon dont plusieurs utilisateurs interagissent, et le tester, est compliqué. Voici la spécification suivante que nous suivons pour le contrôle d'accès,
que vous pouvez utiliser pour tester votre implémentation:

    Page avec IDs de groupe null, utilisateur avec IDs de groupe null - devrait avoir accès.
    Page avec IDs de groupe null, utilisateur avec des IDs de groupe - devrait avoir accès.
    Page avec des IDs de groupe, utilisateur avec IDs de groupe null - devrait avoir accès.
    Page avec des IDs de groupe, utilisateur avec une liste vide - NE devrait PAS avoir accès.
    Page avec des IDs de groupe, utilisateur avec des IDs de groupe - intersection existe - devrait avoir accès.
    Page avec des IDs de groupe, utilisateur avec des IDs de groupe - l'intersection n'existe pas - NE devrait PAS avoir accès.
    Page avec une liste vide d'IDs de groupe (personne n'a accès), utilisateur avec null - NE devrait PAS avoir accès.
    
    Utilisateur SSO A = Aucun ID de groupe défini (null = accès complet).
    Utilisateur SSO B = Aucun ID de groupe défini (null = accès complet).
    A peut @B.
    
    Utilisateur SSO A = Aucun ID de groupe défini (null = accès complet).
    Utilisateur SSO B = IDs de groupe définis.
    A peut @B.
    
    Utilisateur SSO A = IDs de groupe définis.
    Utilisateur SSO B = Aucun ID de groupe défini (null = accès complet).
    A peut @B.
    
    Utilisateur SSO A = IDs de groupe = [a].
    Utilisateur SSO B = IDs de groupe = [b].
    A NE peut PAS @B.
    
    Utilisateur SSO A = IDs de groupe = [a].
    Utilisateur SSO B = IDs de groupe = [a, b].
    A peut @B.