Ajoutez le SDK FastComments au fichier `build.gradle.kts` de votre application :

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Assurez-vous d'avoir le dépôt Repsy dans le fichier `settings.gradle.kts` de votre projet :

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```