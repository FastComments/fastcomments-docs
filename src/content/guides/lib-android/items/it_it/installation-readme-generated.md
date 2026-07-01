Aggiungi il FastComments SDK al file `build.gradle.kts` della tua app:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Assicurati di avere il repository Repsy nel file `settings.gradle.kts` del tuo progetto:

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