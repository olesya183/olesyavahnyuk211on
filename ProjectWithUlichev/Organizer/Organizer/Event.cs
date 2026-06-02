using System; // Підключаємо базові можливості (дати, типи і т.д.)

namespace Organizer // Назва мого проєкту, підключаємо простір імен (наших зміних)
{
    public class Event // Опис-шаблон одного заходу
    {
        public string Name { get; set; } // Назва (імя змінної) заходу

        public string Category { get; set; } // Категорія

        public string Goal { get; set; } // Мета заходу

        public string Organizer { get; set; } // Відповідальна особа

        public string Place { get; set; } // Місце проведення

        public DateTime StartDate { get; set; } // Дата початку

        public DateTime EndDate { get; set; } // Дата завершення

        public string Status { get; set; } // Ступінь виконання (Не почато / В процесі / Завершено)
    }
}
