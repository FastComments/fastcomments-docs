Fügen Sie das FastComments SDK zur `build.gradle.kts`-Datei Ihrer App hinzu:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Stellen Sie sicher, dass Sie das Repsy-Repository in der `settings.gradle.kts`-Datei Ihres Projekts haben:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // weitere Repositories...
    }
}
```