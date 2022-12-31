job "test" {
  type = "batch"

  datacenters = ["dc1"]

  parameterized {
    payload = "required"
    meta_required = ["count", "name"]
    meta_optional = ["bar"]
  }

  group "hello" {
    count = 1

    task "test" {
      driver = "docker"

      config {
        image = "bash:latest"

        args = [
          "echo",
          "${counter}",
          "${name}",
          "optional",
          "${bar}"
        ]
      }

      env {
        counter = "${NOMAD_META_COUNT}"
        name = "${NOMAD_META_NAME}" 
        bar  = "${NOMAD_META_BAR}"
      }
    }
  }
}
