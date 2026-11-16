Aggiungi l'SDK FastComments al file `build.gradle.kts` della tua app:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Assicurati di avere il repository Repsy nel file `settings.gradle.kts` del tuo progetto:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // altri repository...
    }
}
```