### Maven

Ajoutez le dépôt Repsy au POM de votre projet :

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Ensuite, ajoutez les dépendances dont vous avez besoin :

```xml
<dependencies>
    <!-- Client API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Bibliothèque Core (inclut SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Bibliothèque PubSub (pour les événements en direct) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Ajoutez le dépôt Repsy à votre fichier build.gradle :

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Client API
    implementation "com.fastcomments:client:2.0.0"
    
    // Bibliothèque Core (inclut SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // Bibliothèque PubSub (pour les événements en direct)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Cette bibliothèque contient trois modules. Le client API généré, la bibliothèque Java core qui contient des utilitaires écrits à la main pour faciliter le travail avec l'API, et le module `pubsub` qui est une bibliothèque pour s'abonner aux flux de modifications.

- [Documentation de la bibliothèque Client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentation de la bibliothèque Core, incluant des exemples SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentation de la bibliothèque PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Pour le client API, il y a trois classes, `DefaultApi`, `PublicApi`, et `ModerationApi`. `DefaultApi` contient des méthodes qui nécessitent votre clé API, et `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc. sans authentification.

La `ModerationApi` alimente le tableau de bord des modérateurs. Elle contient des méthodes pour la modération des commentaires (liste, comptage, recherche, journaux et export), les actions de modération (supprimer/restaurer, signaler, définir l'état de révision/spam/approbation, votes, et réouverture/fermeture de fil), les bannissements (bannir d'un commentaire, annuler une interdiction, résumés pré-bannissement, statut et préférences de bannissement, et compte des utilisateurs bannis), et les badges et la confiance (attribuer/retirer un badge, badges manuels, obtenir/définir le facteur de confiance, et profil interne de l'utilisateur). Chaque méthode de `ModerationApi` accepte un paramètre `sso` afin que l'appel puisse être effectué au nom d'un modérateur authentifié via SSO.