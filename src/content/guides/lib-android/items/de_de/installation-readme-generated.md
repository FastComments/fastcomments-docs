Fügen Sie das FastComments SDK zu der `build.gradle.kts`‑Datei Ihrer App hinzu:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Stellen Sie sicher, dass Sie das Repsy‑Repository in den `settings.gradle.kts` Ihres Projekts haben:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // andere Repositorys...
    }
}
```