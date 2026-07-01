Ajoutez le SDK FastComments au fichier `build.gradle.kts` de votre application :

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Assurez‑vous que le dépôt Repsy est présent dans le `settings.gradle.kts` de votre projet :

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // autres dépôts...
    }
}
```