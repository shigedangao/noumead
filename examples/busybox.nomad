job "busybox" {
  type = "batch"

  datacenters = ["dc1"]

  parameterized {
    payload = "required"
    meta_required = ["word"]
  }

  group "hello" {
    count = 1

    task "test" {
      driver = "docker"

      config {
        image = "busybox:1.28"

        args = [
          "-c",
          "echo ${word}; sleep 5s; echo ${word} lala; sleep 5s; echo yay; sleep 5s;"
        ]

        command = "/bin/sh"
      }

      env {
        word = "${NOMAD_META_WORD}"
      }
    }
  }
}
