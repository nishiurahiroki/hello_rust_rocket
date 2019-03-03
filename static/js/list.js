document
  .getElementsByName('delete_button')
  .forEach(button => button.addEventListener('click', e => {
    document.getElementById('target_todo_id').value = e.target.dataset.targetTodoId

    const form = document.getElementById('refineForm')
    form.action = './delete'
    form.method = 'post'
    form.submit()
  }))

document
  .getElementsByName('detail_button')
  .forEach(button => button.addEventListener('click', e => {
    document.getElementById('target_todo_id').value = e.target.dataset.targetTodoId

    const form = document.getElementById('refineForm')
    form.action = `./detail`
    form.method = 'get'
    form.submit()
  }))
