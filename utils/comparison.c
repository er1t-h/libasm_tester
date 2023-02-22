int compare_first_letter(const void *s1, const void *s2) {
	const char *ns1 = s1;
	const char *ns2 = s2;
	return *ns1 - *ns2;
}
