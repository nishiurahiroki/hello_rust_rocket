document
  .getElementsByName('delete_button')
  .forEach(button => button.addEventListener('click', e => {
    document.getElementById('target_todo_id').value = e.target.dataset.targetTodoId

    const searchForm = document.getElementById('searchForm')
    searchForm.action = './delete'
    searchForm.method = 'post'
    searchForm.submit()
  }))

document
  .getElementsByName('detail_button')
  .forEach(button => button.addEventListener('click', e => {
    const searchForm = document.getElementById('searchForm')
    searchForm.action = `./detail?todo_id=${e.target.dataset.targetTodoId}`
    searchForm.method = 'get'
    searchForm.submit()
  }))
