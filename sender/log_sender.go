package log_sender

import (
	"encoding/json"
	"fmt"
	"net"
)

type Log struct {
	LogType    string         `json:"log_type"`
	Message    string         `json:"message"`
	Timestamp  string         `json:"timestamp"`
	Attributes []LogAttribute `json:"attributes"`
	Index      string         `json:"index"`
}

type LogAttribute struct {
	Name  string `json:"name"`
	Value string `json:"value"`
}

func LogInfo(message string, attributes ...LogAttribute) error {
	// the index must come from a config file
	log := Log{
		LogType:    "info",
		Message:    message,
		Timestamp:  "",
		Attributes: attributes,
		Index:      "",
	}

	return send_log(log)
}

func LogError(message string, attributes ...LogAttribute) error {
	log := Log{
		LogType:    "error",
		Message:    message,
		Timestamp:  "",
		Attributes: attributes,
		Index:      "",
	}

	return send_log(log)
}

func send_log(log Log) error {
	socket, err := net.Dial("unix", "/tmp/rst.sock")

	if err != nil {
		fmt.Println("unable to connect to socket. Err: %v", err.Error())
		return err
	}

	defer socket.Close()

	serializedLog, err := json.Marshal(log)

	fmt.Println("data: ", log)
	fmt.Println("parsed data: ", serializedLog)

	if err != nil {
		fmt.Println("unable to serialize log data. Err: %v", err.Error())
		return err
	}

	_, err = socket.Write(serializedLog)

	if err != nil {
		fmt.Println("unable to send log data. Err: %v", err.Error())
	}

	return err
}
