document
  .getElementsByName('delete_button')
  .forEach(button => button.addEventListener('click', e => {
    document.getElementById('target_todo_id').value = e.target.dataset.targetTodoId

    const searchForm = document.getElementById('searchForm')
    searchForm.action = './delete'
    searchForm.method = 'post'
    searchForm.submit()
  }))
