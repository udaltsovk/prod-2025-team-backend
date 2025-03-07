package service

import (
	"fmt"
	"gitlab.com/drop-table-prod/backend/services/go/mail/internal/domain/utils/dotenv"
	"net/smtp"
)

// mailService представляет сервис для отправки писем
type mailService struct {
	from     string
	password string
	smtpHost string
	smtpPort string
}

// NewMailService создает новый экземпляр mailService
func NewMailService(from, password, smtpHost, smtpPort string) mailService {
	return mailService{
		from:     from,
		password: password,
		smtpHost: smtpHost,
		smtpPort: smtpPort,
	}
}

// SendEmail отправляет письмо на указанный адрес
func (s mailService) SendEmail(to []string, topic, body string) error {
	// Формируем сообщение с правильными заголовками и телом
	message := fmt.Sprintf("To: %s\r\nSubject: %s\r\n\r\n%s", to[0], topic, body)

	// Аутентификация
	auth := smtp.PlainAuth("", s.from, s.password, s.smtpHost)

	// Отправка письма
	err := smtp.SendMail(s.smtpHost+":"+s.smtpPort, auth, s.from, to, []byte(message))
	if err != nil {
		return fmt.Errorf("failed to send email: %w", err)
	}

	return nil
}

func NewMailServiceFromEnv() mailService {
	from := dotenv.GetEnv("MAIL_EMAIL", "")
	password := dotenv.GetEnv("MAIL_PASSWORD", "")
	smtpHost := dotenv.GetEnv("MAIL_HOST", "")
	smtpPort := dotenv.GetEnv("MAIL_PORT", "587")

	return NewMailService(from, password, smtpHost, smtpPort)
}
