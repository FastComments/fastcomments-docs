Voeg de FastComments SDK toe aan het `build.gradle.kts` bestand van je app:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Zorg ervoor dat je de Repsy repository in het `settings.gradle.kts` bestand van je project hebt:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // andere repositories...
    }
}
```