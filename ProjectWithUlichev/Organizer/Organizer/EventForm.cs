using System;
using System.Windows.Forms;

namespace Organizer
{
    public partial class EventForm : Form
    {

        //Поле для передачі даних
        public Event CurrentEvent { get; set; } // Тут зберігається подія       
        
        //Конструктор 1 (для додавання)
        public EventForm()
        {
            InitializeComponent(); // Ініціалізація форми

            cmbStatus.SelectedIndex = 0; // За замовчуванням "Не почато"
        }

        //Конструктор 2 (для редагування)
        public EventForm(Event ev)
        {
            InitializeComponent(); // Ініціалізація

            CurrentEvent = ev; // Запам'ятовуємо подію

            // Заповнюємо поля значеннями
            txtName.Text = ev.Name;
            cmbCategory.Text = ev.Category;
            txtGoal.Text = ev.Goal;
            txtOrganizer.Text = ev.Organizer;
            txtPlace.Text = ev.Place;

            dateStart.Value = ev.StartDate;
            dateEnd.Value = ev.EndDate;

            cmbStatus.Text = ev.Status;
        }

        //Кнопка OK
        private void btnOK_Click(object sender, EventArgs e)
        {
            // Якщо це нова подія — створюємо її
            if (CurrentEvent == null)
            {
                CurrentEvent = new Event();
            }

            // Записуємо дані з форми в об'єкт
            CurrentEvent.Name = txtName.Text; // Назва
            CurrentEvent.Category = cmbCategory.Text; // Категорія
            CurrentEvent.Goal = txtGoal.Text; // Мета
            CurrentEvent.Organizer = txtOrganizer.Text; // Організатор
            CurrentEvent.Place = txtPlace.Text; // Місце

            CurrentEvent.StartDate = dateStart.Value; // Дата початку
            CurrentEvent.EndDate = dateEnd.Value; // Дата кінця

            CurrentEvent.Status = cmbStatus.Text; // Ступінь виконання
           
            //Перевірка 1 чи поле Назва заходу заповнене
            if (string.IsNullOrWhiteSpace(txtName.Text))
            {
                MessageBox.Show("Введіть назву!");
                return;
            }
            //Перевірка 2 чи дата Заверешення події не менша за дату початку
            if (dateEnd.Value < dateStart.Value)
            {
                MessageBox.Show("Дата завершення не може бути раніше початку!");
                return;
            }

            this.DialogResult = DialogResult.OK; // Закриває форму як "OK"
        }

        //Кнопка Cancel
        private void btnCancel_Click(object sender, EventArgs e)
        {
            this.DialogResult = DialogResult.Cancel; // Просто закриває
        }


    }
}
