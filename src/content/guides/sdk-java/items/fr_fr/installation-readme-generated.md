### Maven

Ajoutez le dépôt Repsy au POM de votre projet:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Then add the dependencies you need:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Ajoutez le dépôt Repsy à votre fichier build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Cette bibliothèque contient trois modules. Le client API généré, la bibliothèque Java core qui contient des utilitaires écrits à la main pour faciliter le travail avec l'API, et le module `pubsub` qui est une bibliothèque pour s'abonner aux flux de changements.

- [Docs de la bibliothèque client API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Docs de la bibliothèque Core, y compris des exemples SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Docs de la bibliothèque PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Pour le client API, il existe trois classes, `DefaultApi`, `PublicApi`, et `ModerationApi`. La `DefaultApi` contient des méthodes qui nécessitent votre clé API, et la `PublicApi` contient des méthodes qui peuvent être appelées directement depuis un navigateur/appareil mobile/etc sans authentification.

La `ModerationApi` alimente le tableau de bord des modérateurs. Elle contient des méthodes pour la modération des commentaires (liste, comptage, recherche, journaux et export), les actions de modération (supprimer/restaurer, signaler, définir le statut revue/spam/approbation, votes, et rouvrir/fermer un fil), les bannissements (interdire de commenter, annuler une interdiction, résumés pré-interdiction, statut et préférences de bannissement, et nombre d'utilisateurs bannis), et les badges & la confiance (attribuer/supprimer un badge, badges manuels, obtenir/définir le facteur de confiance, et profil interne de l'utilisateur). Chaque méthode de la `ModerationApi` accepte un paramètre `sso` afin que l'appel puisse être effectué au nom d'un modérateur authentifié via SSO.