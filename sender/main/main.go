package main

import log_sender "github.com/raonismaneoto/rlogger/sender"

func main() {
	attribute := log_sender.LogAttribute{
		Name:  "host",
		Value: "raoni-pc",
	}

	log_sender.LogInfo("testiing", attribute)
}
