#pragma once

#include <sys/types.h>

typedef struct s_list
{
	void			*data;
	struct s_list	*next;
}					t_list;

// Mandatory
size_t	ft_strlen(const char *str);
char	*ft_strcpy(char *dest, const char *src);
int		ft_strcmp(const char *str1, const char *str2);
ssize_t	ft_write(int fd, const void *buffer, size_t count);
ssize_t	ft_read(int fd,  void *buffer, size_t count);
char	*ft_strdup(const char *str);

// Bonus
int		ft_atoi_base(const char *str, const char *base);
void	ft_list_push_front(t_list **begin, void *data);
size_t	ft_list_size(t_list *begin);
void	ft_list_remove_if(t_list **begin_list, void *data_ref, int (*cmp)(const void *, const void *), void (*free_fct)(void *));
void	ft_list_sort(t_list **begin_list, int (*cmp)());

// Utils
int		compare_first_letter(const void *s1, const void *s2);
void	no_free(void *mem);
