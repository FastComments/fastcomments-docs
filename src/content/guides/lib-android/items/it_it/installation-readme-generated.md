Aggiungi l'SDK di FastComments al file `build.gradle.kts` della tua app:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Assicurati di avere il repository Repsy nel `settings.gradle.kts` del tuo progetto:

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