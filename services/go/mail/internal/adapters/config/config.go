package config

import (
	"github.com/spf13/viper"
	"gitlab.com/drop-table-prod/backend/services/go/mail/internal/adapters/logger"
	"log"
)

type Config struct {
}

func initConfig() {
	viper.SetConfigName("config")
	viper.SetConfigType("yaml")
	viper.AddConfigPath("services/go/mail/")

	if err := viper.ReadInConfig(); err != nil {
		log.Panicf("failed to read config: %v", err)
	}
}

func Configure() *Config {

	logger.New(
		true,
		"GMT+3",
	)
	logger.Log.Debugf("Debug mode: %t", viper.GetBool("settings.debug"))

	logger.Log.Info("Configure success")

	return &Config{}
}
