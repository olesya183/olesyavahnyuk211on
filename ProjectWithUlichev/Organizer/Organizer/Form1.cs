using System;
using System.Collections.Generic; // Для List
using System.Windows.Forms;
using System.IO;
using Newtonsoft.Json;//прийшлось встановити додаткову бібліотеку



namespace Organizer
{
    public partial class Form1 : Form
    {

        List<Event> events = new List<Event>(); // Список всіх заходів

        public Form1()
        {
            InitializeComponent();
            //при відкриті та закритті ЦІЄЇ форми спрацюють методи з меседжбоксами
            this.Shown += Form1_Shown;
            this.FormClosing += Form1_FormClosing;


            //код, щоб стовпці таблиці не створювалися автоматично, а були визначені нами
            dataGridViewEvents.AutoGenerateColumns = false;
        }

        //меседжбокс при відкритті форми
        private void Form1_Shown(object sender, EventArgs e)
        {
            MessageBox.Show("Не забудьте завантажити дані!");
        }

        //меседжбокс при закритті форми, плюс збереження, якщо обрали
        private void Form1_FormClosing(object sender, FormClosingEventArgs e)
        {
            DialogResult result = MessageBox.Show(
                "Бажаєте зберегти дані перед виходом?",
                "Збереження",
                MessageBoxButtons.YesNoCancel);

            if (result == DialogResult.Yes)
            {
                btnSave_Click(null, null); // Зберегти
            }
            else if (result == DialogResult.Cancel)
            {
                e.Cancel = true; // Не закривати
            }
        }

        //Відображення списку
        private void RefreshGrid() // Оновлює таблицю
        {
            dataGridViewEvents.DataSource = null; // Очищаємо прив'язку
            dataGridViewEvents.DataSource = events; // Підключаємо список

            // Це змушує колонки розтягнутися по ширині
            dataGridViewEvents.AutoSizeColumnsMode = DataGridViewAutoSizeColumnsMode.Fill;
        }

        //Кнопка "Додати"
        private void btnAdd_Click(object sender, EventArgs e)
        {
            EventForm form = new EventForm(); // Створюємо форму

            if (form.ShowDialog() == DialogResult.OK) // Якщо натиснули OK
            {
                events.Add(form.CurrentEvent); // Додаємо подію в список

                RefreshGrid(); // Оновлюємо таблицю
            }
        }


        //Кнопка "Редагувати"
        private void btnEdit_Click(object sender, EventArgs e)
        {
            if (dataGridViewEvents.CurrentRow == null) return; // Якщо нічого не вибрано

            Event selectedEvent = (Event)dataGridViewEvents.CurrentRow.DataBoundItem; // Беремо вибраний захід

            EventForm form = new EventForm(selectedEvent); // Передаємо його у форму

            if (form.ShowDialog() == DialogResult.OK)
            {
                RefreshGrid(); // Оновлюємо таблицю
            }
        }

        //Кнопка "Видалити"
        private void btnDelete_Click(object sender, EventArgs e)
        {
            if (dataGridViewEvents.CurrentRow == null) return;

            Event selectedEvent = (Event)dataGridViewEvents.CurrentRow.DataBoundItem;

            events.Remove(selectedEvent); // Видаляємо

            RefreshGrid(); // Оновлюємо
        }

        //Пошук по назві
        private void btnSearchName_Click(object sender, EventArgs e)
        {
            string search = txtSearchName.Text.ToLower(); // Беремо текст

            var result = events.FindAll(ev => ev.Name.ToLower().Contains(search));

            dataGridViewEvents.DataSource = result; // Показуємо результат
        }

        //Пошук по даті
        private void btnSearchDate_Click(object sender, EventArgs e)
        {
            DateTime selectedDate = dateSearch.Value.Date;

            var result = events.FindAll(ev => ev.StartDate.Date == selectedDate);

            dataGridViewEvents.DataSource = result;
        }

        // Кнопка скинути пошук
        private void btnReset_Click(object sender, EventArgs e)
        {
            txtSearchName.Clear(); // Очищуємо поле пошуку за назвою
            RefreshGrid();
        }

        //Кнопка Зберегти формат JSON (збереження)
        private void btnSave_Click(object sender, EventArgs e)
        {
            string json = JsonConvert.SerializeObject(events); // Перетворюємо список в JSON

            File.WriteAllText("events.json", json); // Зберігаємо у файл
        }


        //Кнопка Завантажити данні з файлу events.json формат JSON (завантаження)
        private void btnLoad_Click(object sender, EventArgs e)
        {
            if (File.Exists("events.json")) // Якщо файл існує
            {
                string json = File.ReadAllText("events.json"); // Читаємо

                events = JsonConvert.DeserializeObject<List<Event>>(json); ; // Перетворюємо назад у список

                RefreshGrid(); // Оновлюємо таблицю
            }
        }

    }
}
