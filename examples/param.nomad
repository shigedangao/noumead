job "example" {
  type = "batch"

  datacenters = ["dc1"]

  parameterized {
    payload = "required"
    meta_required = ["count", "name"]
  }

  group "hello" {
    count = 1

    task "test" {
      driver = "docker"

      config {
        image = "bash:latest"

        args = [
          "echo",
          "hello ${counter}",
          "${name}"
        ]
      }

      env {
        counter = "${NOMAD_META_COUNT}"
        name = "${NOMAD_META_NAME}" 
      }
    }
  }
}